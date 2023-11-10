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

    let mut state: game1::State = glider.to_string().into();

    // for _ in 0..50 {
    //     println!("{}\n", state);
    //     state.tick();
    // }
    println!("{}\n", state);
    state.tick();
    println!("{}\n", state);
}
