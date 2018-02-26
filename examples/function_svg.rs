extern crate plotlib;

fn main() {
    let f1 = plotlib::function::Function::new(|x| x * 5., 0., 10.)
        .style(plotlib::function::Style::new().colour("burlywood"));
    let f2 = plotlib::function::Function::new(|x| x.powi(2), 0., 10.)
        .style(plotlib::function::Style::new().colour("darkolivegreen"));
    let f3 = plotlib::function::Function::new(|x| x.sqrt() * 20., 0., 10.)
        .style(plotlib::function::Style::new().colour("brown"));
    let v = plotlib::view::View::new().add(&f1).add(&f2).add(&f3);
    plotlib::page::Page::single(&v).save("function.svg");
}