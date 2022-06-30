use wasmedge_tensorflow_interface;
use std::io::{self, Read};
use std::time::{Instant};

pub fn main() {
    let start = Instant::now();
    let model_data: &[u8] = include_bytes!("models/birds/lite-model_aiy_vision_classifier_birds_V1_3.tflite");
    let labels = include_str!("models/birds/aiy_birds_V1_labelmap.txt");

    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).unwrap();

    let mut flat_img = wasmedge_tensorflow_interface::load_jpg_image_to_rgb32f(&buf, 224, 224);
    flat_img.iter_mut().for_each(|x| *x *= 1.0/255.0);

    let mut session = wasmedge_tensorflow_interface::Session::new(&model_data, wasmedge_tensorflow_interface::ModelType::TensorFlowLite);
    session.add_input("input", &flat_img, &[1, 224, 224, 3])
           .run();
    let res_vec: Vec<f32> = session.get_output("MobilenetV2/Predictions/Softmax");
    println!("Parsed output in ... {:?}", start.elapsed());

    let mut i = 0;
    let mut max_index: i32 = -1;
    let mut max_value: f32 = -1.0;
    while i < res_vec.len() {
        let cur = res_vec[i];
        if cur > max_value {
            max_value = cur;
            max_index = i as i32;
        }
        i += 1;
    }

    let mut label_lines = labels.lines();
    for _i in 0..max_index {
      label_lines.next();
    }
    println!("{}", label_lines.next().unwrap().to_string());
}