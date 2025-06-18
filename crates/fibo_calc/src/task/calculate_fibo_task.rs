use crate::{FiboBuilder, FiboTaskResult};


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
        let progress = ((processed as f64 / total_items as f64) * 100.0) as u8;
        let _ = sender.send(FiboTaskResult::Calculation(progress));
    }

    if range.start <= 1 && range.end > 1 {
        result.push(start_nums.1.clone());
        processed += 1;
        let progress = ((processed as f64 / total_items as f64) * 100.0) as u8;
        let _ = sender.send(FiboTaskResult::Calculation(progress));
    }

    if range.end > 2 {
        let implementation_fibo = crate::implementation::lineal::LinealFibo::new(Some(start_nums));
        let skip_count = if range.start > 2 { range.start - 2 } else { 0 };
        let take_count = range.end - 2;

        for num in implementation_fibo.skip(skip_count).take(take_count) {
            result.push(num);
            processed += 1;

            if processed % 10 == 0 || processed == total_items {
                let progress = ((processed as f64 / total_items as f64) * 100.0) as u8;
                let _ = sender.send(FiboTaskResult::Calculation(progress));
            }
        }
    }

    let filters = builder.get_filters();
    let filtered_result: Vec<_> = result
        .into_iter()
        .filter(|n| filters.iter().all(|func| func(n)))
        .collect();

    let _ = sender.send(FiboTaskResult::Calculation(100));

    let _ = sender.send(FiboTaskResult::Result(filtered_result));
}
