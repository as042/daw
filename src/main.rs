use daw::prelude::*;
#[allow(unused_imports)]
use performance_tester::test_performance;

fn main() {
    project_to_wav()
}

fn project_to_wav() {
    let mut path = String::default();
    std::io::stdin().read_line(&mut path).uw();

    let from_toml = Project::from_toml(path.trim(), true);
    if let Ok(output) = from_toml {
        output.0.export_wav(output.1, output.2, true).uw();
    }  
    else {
        println!("Error: {}. \nOperation unsuccessful.", from_toml.unwrap_err());
    }

    std::io::stdin().read_line(&mut path).uw();
}