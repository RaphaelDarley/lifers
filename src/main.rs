use lifers::basic_swap;
use lifers::game1;
use lifers::Game;

fn main() {
    let mut state = game1::State {
        width: 3,
        height: 3,
        board: vec![
            vec![false, false, false],
            vec![true, true, true],
            vec![false, false, false],
        ],
    };

    let glider = r#"00000
00&00
&0&00
0&&00
00000"#;

    let big_glider = r#"````````````````````
``&`````````````````
&`&`````````````````
`&&`````````````````
````````````````````
````````````````````
````````````````````
````````````````````
````````````````````
````````````````````
````````````````````
````````````````````
````````````````````
````````````````````
````````````````````
````````````````````
"#;

    // println!("{:?}", glider);

    let mut g1_state: game1::State = glider.to_string().into();
    let mut swp_state: basic_swap::State = glider.to_string().into();

    println!("{:?}", swp_state.current);
    println!("{:?}", g1_state.board);

    println!("{swp_state}\n");
    println!("{g1_state}\n");
    swp_state.tick();
    g1_state.tick();
    // println!("{swp_state}\n");
    // println!("{g1_state}\n");

    // for _ in 0..10 {
    //     println!("{}\n", state);
    //     state.tick();
    // }
    // println!("{}\n", state);
    // state.tick();
    // println!("{}\n", state);
}
