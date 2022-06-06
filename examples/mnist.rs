use mnist::{Mnist, MnistBuilder};
use rand::Rng;
use rulinalg::matrix::{BaseMatrix, Matrix};
use std::{env, path::PathBuf};

fn load_mnist(
    size: u32,
    rows: u32,
    cols: u32,
    img: &Vec<u8>,
    lbl: &Vec<u8>,
    index: usize,
) -> (u8, Matrix<u8>) {
    let img = Matrix::new((size * rows) as usize, cols as usize, img.clone());
    let s = index * 28;
    let e = s + 28;
    let row_indexes = (s..e).collect::<Vec<_>>();
    let img = img.select_rows(&row_indexes);

    (lbl[index], img)
}

fn save_model(size: u32, img: &Vec<u8>, lbl: &Vec<u8>, rows: u32, cols: u32) -> bool {
    println!("Load mnist data.");
    let annoy = rannoy::Rannoy::new(28 * 28);

    for i in 0..size {
        let (_, img) = load_mnist(size, rows, cols, &img, &lbl, i as usize);

        let img_to_vec = img
            .data()
            .clone()
            .into_iter()
            .map(|v| v as f32)
            .collect::<Vec<_>>();

        annoy.add_item(i, &img_to_vec);

        if i % 1_000 == 0 {
            println!("Add item {}/{}.", i, size);
        }
    }

    annoy.build(30);

    annoy.save(PathBuf::from("mnist.ann"))
}

fn main() {
    let (trn_size, tst_size, rows, cols) = (10_000, 10_000, 28, 28);
    let Mnist {
        trn_img,
        trn_lbl,
        tst_img,
        tst_lbl,
        ..
    } = MnistBuilder::new()
        .label_format_digit()
        .training_set_length(trn_size)
        .test_set_length(tst_size)
        .finalize();

    let mut ann_file = env::current_dir().unwrap();
    ann_file.push("mnist.ann");

    if !ann_file.exists() {
        if !save_model(trn_size, &trn_img, &trn_lbl, rows, cols) {
            eprintln!("failed to save index file");
            return;
        }
    }

    let annoy = rannoy::Rannoy::new(28 * 28);
    if !annoy.load(ann_file) {
        eprintln!("failed to load index file");
        return;
    }

    let mut rng = rand::thread_rng();
    for i in 0..10 {
        let ti: u32 = rng.gen();
        let (lbl, img) = load_mnist(
            trn_size,
            rows,
            cols,
            &tst_img,
            &tst_lbl,
            (ti % tst_size) as usize,
        );
        let img_to_vec = img
            .data()
            .clone()
            .into_iter()
            .map(|v| v as f32)
            .collect::<Vec<_>>();

        let (result, _distance) = annoy.get_nns_by_vector(&img_to_vec, 3, -1);
        let actual = result
            .iter()
            .map(|&v| trn_lbl[v as usize])
            .collect::<Vec<_>>();

        println!("TEST{}: expected: {}, actual: {:?}", i, lbl, actual);
        if actual[0] != lbl {
            let (_, trn) = load_mnist(trn_size, rows, cols, &trn_img, &trn_lbl, result[0] as usize);

            println!("{}\n{}", img, trn);
        }
    }
}
