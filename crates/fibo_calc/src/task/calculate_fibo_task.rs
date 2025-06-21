use crate::builder::FilterFn;
use crate::implementation::lineal::LinealFibo;
use crate::{FiboBuilder, FiboTaskResult};
use num_bigint::BigInt;
use rayon::prelude::*;

const CHUNK_SIZE: usize = 1000;


pub async fn calculate_fibo_task(builder: FiboBuilder, sender: crate::task::FiboTaskSender) {
    if builder.is_none_filter() {
        let _ = sender.send(FiboTaskResult::Result(vec![]));
        return;
    }

    let (start_nums, range) = match (builder.get_start_nums(), builder.get_range_by_id()) {
        (Some((n1, n2)), Some(range)) if range.start <= range.end => ((n1, n2), range),
        _ => {
            let _ = sender.send(FiboTaskResult::Result(vec![]));
            return;
        }
    };

    let total_items = range.end.saturating_sub(range.start);
    if total_items == 0 {
        let _ = sender.send(FiboTaskResult::Result(vec![]));
        return;
    }

    let mut result = Vec::with_capacity(total_items);
    let mut processed = 0;

    if range.start == 0 {
        result.push(start_nums.0.clone());
        processed += 1;
        send_progress(&sender, processed, total_items).await;
    }

    if range.start <= 1 && range.end > 1 {
        result.push(start_nums.1.clone());
        processed += 1;
        send_progress(&sender, processed, total_items).await;
    }

    if range.end > 2 {
        let implementation_fibo = LinealFibo::new(Some(start_nums));
        let skip_count = if range.start > 2 { range.start - 2 } else { 0 };
        let take_count = range.end - 2;

        for num in implementation_fibo.skip(skip_count).take(take_count) {
            result.push(num);
            processed += 1;

            if processed % 10 == 0 || processed == total_items {
                send_progress(&sender, processed, total_items).await;
            }
        }
    }

    let filters = builder.get_filters();
    let filtered_result = apply_filters_with_progress(&sender, result, filters).await;

    let _ = sender.send(FiboTaskResult::Result(filtered_result));
}

async fn apply_filters_with_progress(
    sender: &crate::task::FiboTaskSender, numbers: Vec<BigInt>, filters: &[FilterFn],
) -> Vec<BigInt> {
    let total_items = numbers.len();
    if total_items == 0 {
        return numbers;
    }

    if filters.is_empty() {
        send_progress(sender, total_items, total_items).await;
        return numbers;
    }

    let mut filtered = Vec::with_capacity(total_items);
    let mut processed = 0;

    for chunk in numbers.chunks(CHUNK_SIZE) {
        let filtered_chunk: Vec<BigInt> = chunk
            .par_iter()
            .filter(|num| filters.iter().all(|f| f(num)))
            .cloned()
            .collect();

        filtered.extend(filtered_chunk);
        processed += chunk.len();

        send_progress(sender, processed, total_items).await;
    }

    filtered
}

async fn send_progress(sender: &crate::task::FiboTaskSender, processed: usize, total_items: usize) {
    let progress = ((processed as f32 / total_items as f32) * 100.0).clamp(0.0, 100.0) as u8;
    let _ = sender.send(FiboTaskResult::Calculation(progress));
}

#[cfg(test)]
mod tests {
    #![warn(clippy::reversed_empty_ranges)]

    use super::*;
    use crate::FiboBuilder;
    use crate::task::{FiboTaskReceiver, FiboTaskResult};
    use num_bigint::BigInt;
    use std::ops::Range;
    use test_case::test_case;
    use tokio::sync::mpsc;

    fn make_sender() -> (crate::task::FiboTaskSender, FiboTaskReceiver) {
        mpsc::unbounded_channel()
    }

    #[tokio::test]
    async fn test_none_filter() {
        let (tx, mut rx) = make_sender();
        let builder = FiboBuilder::default();
        calculate_fibo_task(builder, tx).await;
        let msg = rx.recv().await.unwrap();
        assert!(matches!(msg, FiboTaskResult::Result(ref v) if v.is_empty()));
    }

    #[tokio::test]
    async fn test_invalid_range() {
        let (tx, mut rx) = make_sender();
        let mut builder = FiboBuilder::default();
        builder.set_range_by_id(Some(5..3)); // start > end
        builder.set_start_nums(Some((0.into(), 1.into())));
        calculate_fibo_task(builder, tx).await;
        let msg = rx.recv().await.unwrap();
        assert!(matches!(msg, FiboTaskResult::Result(ref v) if v.is_empty()));
    }

    #[tokio::test]
    async fn test_zero_items() {
        let (tx, mut rx) = make_sender();
        let mut builder = FiboBuilder::default();
        builder.set_range_by_id(Some(2..2)); // empty range
        builder.set_start_nums(Some((0.into(), 1.into())));
        calculate_fibo_task(builder, tx).await;
        let msg = rx.recv().await.unwrap();
        assert!(matches!(msg, FiboTaskResult::Result(ref v) if v.is_empty()));
    }

    #[test_case(0..1, (0, 1), vec![(0, 100)], vec![0]; "start at 0, one item")]
    #[test_case(0..2, (0, 1), vec![(0, 50), (1, 100)], vec![0, 1]; "start at 0, two items")]
    #[test_case(1..2, (0, 1), vec![(1, 100)], vec![1]; "start at 1, one item")]
    #[tokio::test]
    async fn test_small_ranges(
        range: Range<usize>, start: (i32, i32), expected_progress: Vec<(usize, u8)>,
        expected_result: Vec<i32>,
    ) {
        let (tx, mut rx) = make_sender();
        let mut builder = FiboBuilder::default();
        builder.set_range_by_id(Some(range.clone()));
        builder.set_start_nums(Some((start.0.into(), start.1.into())));
        calculate_fibo_task(builder, tx).await;

        let mut progresses = vec![];
        let mut result = None;

        while let Some(msg) = rx.recv().await {
            match msg {
                FiboTaskResult::Calculation(p) => {
                    // Only add progress if it's not a duplicate 100%
                    if progresses.last() != Some(&100) || p != 100 {
                        progresses.push(p);
                    }
                }
                FiboTaskResult::Result(res_vec) => {
                    result = Some(res_vec);
                    break;
                }
            }
        }

        let expected: Vec<BigInt> = expected_result.into_iter().map(Into::into).collect();
        assert_eq!(result.unwrap(), expected);

        let expected_progresses: Vec<u8> = expected_progress.into_iter().map(|(_, p)| p).collect();
        assert_eq!(progresses, expected_progresses);
    }

    #[tokio::test]
    async fn test_range_with_progress_and_filter() {
        let (tx, mut rx) = make_sender();
        let mut builder = FiboBuilder::default();
        builder.set_range_by_id(Some(0..12));
        builder.set_start_nums(Some((0.into(), 1.into())));
        builder.add_filter(|n| n % 2u8 == BigInt::from(0));
        calculate_fibo_task(builder, tx).await;

        let mut progress = vec![];
        let mut result = None;

        while let Some(msg) = rx.recv().await {
            match msg {
                FiboTaskResult::Calculation(p) => {
                    progress.push(p);
                }
                FiboTaskResult::Result(res) => {
                    result = Some(res);
                    break;
                }
            }
        }

        assert!(!progress.is_empty(), "No progress updates received");
        assert_eq!(progress.last(), Some(&100), "Last progress should be 100%");

        let res = result.expect("No result received");
        let expected: Vec<BigInt> = vec![0, 2, 8, 34].into_iter().map(Into::into).collect();
        assert_eq!(res, expected);
    }

    #[tokio::test]
    async fn test_apply_filters_with_progress_empty() {
        let (tx, mut rx) = make_sender();
        let numbers: Vec<BigInt> = vec![];
        let filters: Vec<FilterFn> = vec![];
        let res = apply_filters_with_progress(&tx, numbers.clone(), &filters).await;
        assert_eq!(res, numbers);
        // No progress message expected
        assert!(rx.try_recv().is_err());
    }

    #[tokio::test]
    async fn test_apply_filters_with_progress_no_filters() {
        let (tx, mut rx) = make_sender();
        let numbers: Vec<BigInt> = vec![1.into(), 2.into(), 3.into()];
        let filters: Vec<FilterFn> = vec![];
        let res = apply_filters_with_progress(&tx, numbers.clone(), &filters).await;
        assert_eq!(res, numbers);
        // Should send 100% progress
        let msg = rx.recv().await.unwrap();
        assert!(matches!(msg, FiboTaskResult::Calculation(100)));
    }

    #[tokio::test]
    async fn test_apply_filters_with_progress_with_filters() {
        let (tx, mut rx) = make_sender();
        let numbers: Vec<BigInt> = (0..10).map(Into::into).collect();
        let filters: Vec<FilterFn> = vec![Box::new(|n| n % 2u8 == BigInt::from(0))];
        let res = apply_filters_with_progress(&tx, numbers.clone(), &filters).await;
        let expected: Vec<BigInt> = (0..10).filter(|n| n % 2 == 0).map(Into::into).collect();
        assert_eq!(res, expected);
        // Should send progress at least once
        let msg = rx.recv().await.unwrap();
        assert!(matches!(msg, FiboTaskResult::Calculation(_)));
    }

    #[tokio::test]
    async fn test_send_progress_clamps() {
        let (tx, mut rx) = make_sender();
        // processed > total_items
        send_progress(&tx, 15, 10).await;
        let msg = rx.recv().await.unwrap();
        assert!(matches!(msg, FiboTaskResult::Calculation(100)));
        // processed < 0 (should clamp to 0)
        send_progress(&tx, 0, 10).await;
        let msg = rx.recv().await.unwrap();
        assert!(matches!(msg, FiboTaskResult::Calculation(0)));
    }
}
