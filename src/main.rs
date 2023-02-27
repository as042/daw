use daw::prelude::*;
#[allow(unused_imports)]
use performance_tester::test_performance;

fn main() {
    let mut path = String::default();
    std::io::stdin().read_line(&mut path).uw();

    let output = Project::from_toml(path.trim()).uw();  

    output.0.export_wav(output.1, output.2).uw();
}