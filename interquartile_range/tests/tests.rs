extern crate second_law;

fn new_ucmd() -> second_law::UCommand {
    let mut scene: second_law::Scene = second_law::Scene::new(env!("CARGO_PKG_NAME"));
    scene.subcmd_arg("benchcmp");
    scene.ucmd()
}

#[test]
fn test1() {
    // 6 12 8 10 20 16
    // 5 4 3 2 1 5
    let mut x:Vec<f64> = vec![6, 12, 8, 10, 20, 16];
    let mut f:Vec<f64> = vec![5, 4, 3, 2, 1, 5];
    assert_eq!(ir::run(&mut x, &mut f), 9.0)
}
