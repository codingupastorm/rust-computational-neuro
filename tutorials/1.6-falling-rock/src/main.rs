use plotters::prelude::*;

fn main() {
    let dt = 0.01;
    let tmax = 5.0;
    let a = -10.0;
    let t = (0..=(tmax / dt) as usize)
        .map(|i| i as f64 * dt)
        .collect::<Vec<f64>>();

    chart_analytical(&t, tmax, a);
    chart_simulated(&t, dt, tmax, a);
}

fn chart_analytical(t: &Vec<f64>, tmax: f64, a: f64) {
    let v = t.iter().map(|&ti| a * ti).collect::<Vec<f64>>();
    let y = t.iter().map(|&ti| 0.5 * a * ti * ti).collect::<Vec<f64>>();
    
    print_chart(y, t, tmax, "analytical_y.png");
    print_chart(v, t, tmax, "analytical_v.png");
}

fn chart_simulated(t: &Vec<f64>, dt: f64, tmax: f64, a: f64) {
    let mut v_sim = vec![0.0; t.len()];
    let mut y_sim = vec![0.0; t.len()];

    for i in 1..t.len() {
        v_sim[i] = v_sim[i - 1] + dt * a;
        y_sim[i] = y_sim[i - 1] + dt * 0.5 * (v_sim[i - 1] + v_sim[i]);
    }

    print_chart(y_sim, t, tmax, "simulated_y.png");
    print_chart(v_sim, t, tmax, "simulated_v.png");
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