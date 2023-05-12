use plotters::prelude::*;

fn main() {
    let dt = 0.001;
    let tmax = 100.0;
    let t = (0..=(tmax / dt) as usize)
        .map(|i| i as f64 * dt)
        .collect::<Vec<f64>>();

    let resting_potential = -70.0;
    let membrane_resistance = 5.0;
    let membrane_capacity = 2.0;
    let input_voltage = 4.1;

    let spike_threshold = -50.0;
    let reset_potential = -65.0;

    let mut membrane_potential = vec![0.0; t.len()];
    let mut applied_current = vec![input_voltage; t.len()];
    membrane_potential[0] = resting_potential;

    let threshold = (spike_threshold - resting_potential) / membrane_resistance;

    for i in 1..t.len() {
        let input_current = applied_current[i];
        let prev = membrane_potential[i-1];
        let dv_dt = (((resting_potential - prev) / membrane_resistance) + input_current)/membrane_capacity;
        membrane_potential[i] = membrane_potential[i-1] + dv_dt * dt;

        if (membrane_potential[i] > spike_threshold) {
            membrane_potential[i] = reset_potential;
        }
    }

    print_chart(membrane_potential, &t, tmax, "neuron.png"); 
}

fn print_chart(data: Vec<f64>, t: &Vec<f64>, tmax: f64, title: &str) {
    let root = BitMapBackend::new(title, (1200, 800)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let min_range = data
            .iter()
            .cloned()
            .fold(0.0 / 0.0, f64::min);
        let max_range = data
            .iter()
            .cloned()
            .fold(0.0 / 0.0, f64::max);

        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("Arial", 20).into_font())
            .margin(5)
            .set_all_label_area_size(50)
            .build_cartesian_2d(0f64..tmax, min_range..max_range)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                t.iter().cloned().zip(data.iter().cloned()),
                &BLUE,
            ))
            .unwrap();
}