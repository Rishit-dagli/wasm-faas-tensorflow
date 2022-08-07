import {Image} from 'image';
import * as std from 'std';
import {TensorflowLiteSession} from 'tensorflow_lite';

let img = new Image('haemorhous_cassinii.jpg');
let img_rgb = img.to_rgb().resize(192, 192);
let rgb_pix = img_rgb.pixels();

let session = new TensorflowLiteSession(
    'lite-model_aiy_vision_classifier_birds_V1_3.tflite');
session.add_input('input', rgb_pix);
session.run();
let output = session.get_output('MobilenetV2/Predictions/Softmax');
let output_view = new Uint8Array(output);
let max = 0;
let max_idx = 0;
for (var i in output_view) {
  let v = output_view[i];
  if (v > max) {
    max = v;
    max_idx = i;
  }
}
let label_file = std.open('aiy_birds_V1_labelmap.txt', 'r');
let label = '';
for (var i = 0; i <= max_idx; i++) {
  label = label_file.getline();
}
label_file.close();

print('label:');
print(label);

