use rand::Rng;

fn calculate_skew(state: &[bool]) -> f32 {
    4.0
}

fn calculate_contiguous(state: &[bool]) -> f32 {
    2.0
}

fn calculate_fitness(skew: f32, sw: f32, contiguous_value: f32, cw: f32) -> u32 {
    // This is a rubish formula to illustrate the different ways of applying our aspects
    const FORM_CONST: u32 = 50;
    (contiguous_value.powf(cw) + (skew * sw) + FORM_CONST as f32) as u32
}

pub fn assess_fitness(state: &[bool]) -> i32 {
    let skew_value = calculate_skew(state);
    let contiguous_value = calculate_contiguous(state);
    const CONTIGUOUS_WEIGHT: f32 = 1.0;
    const SKEW_WEIGHT: f32 = 1.0;
    calculate_fitness(skew_value, SKEW_WEIGHT, contiguous_value, CONTIGUOUS_WEIGHT)
        .try_into()
        .unwrap()
}

fn select_next_state(state: &[bool]) -> Vec<bool> {
    let original_fitness_score = assess_fitness(&state);
    let next_allocation = select_next_allocation(&state);
    let mut next_state: Vec<bool> = state.into();
    next_state[next_allocation] = true;
    next_state.into()
}

fn select_next_allocation(state: &[bool]) -> usize {
    // Dumb implementation that's random
    let mut rng = rand::thread_rng();
    rng.gen_range(0..state.len())
}

pub fn do_logic(state: &[bool]) -> Vec<bool> {
    // TODO: use actual state
    let demo_month = [
        false, false, false, false, false, true, true, false, false, false, false, false, true,
        true, false, false, false, false, false, true, true, false, false, false, false, false,
        true, true, false, false, false,
    ];

    let current_state = demo_month;
    for days_allocated in 0..5 {
        let current_state = select_next_state(&current_state);
    }
    current_state.into()
}
