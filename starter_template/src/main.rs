use dlib_face_recognition::{FaceDetectorCnn, FaceDetectorTrait, FaceEncoderNetwork, FaceEncoderTrait, FaceEncoding, ImageMatrix, LandmarkPredictor, LandmarkPredictorTrait};

fn get_encoding(file: &str, cnn_detector: &FaceDetectorCnn, landmark_predictor: &LandmarkPredictor, face_encoder_network: &FaceEncoderNetwork) -> FaceEncoding {
    println!("Processing {file}...");

    let now = std::time::Instant::now();

    let photo = image::open(file).expect("Failed to open photo file").to_rgb8();

    let matrix_photo = ImageMatrix::from_image(&photo);

    let face_locations = cnn_detector.face_locations(&matrix_photo);

    let face = face_locations.first().expect("Failed to get face");

    let landmarks =landmark_predictor.face_landmarks(&matrix_photo, face);

    let found_encodings = face_encoder_network.get_face_encodings(&matrix_photo, &[landmarks], 0);

    let face_encoding = found_encodings.first().expect("Failed to get face_encodings");

    println!("[{file}] elapsed time: {}ms\n", now.elapsed().as_millis());

    face_encoding.to_owned()
}

fn main() {
    let Ok(cnn_detector) = FaceDetectorCnn::open("./starter_template/models/mmod_human_face_detector.dat") else {
    //let Ok(cnn_detector) = FaceDetectorCnn::open("../models/mmod_human_face_detector.dat") else {
        panic!("Unable to load cnn face detector!");
    };

    let Ok(landmark_predictor) = LandmarkPredictor::open("./starter_template/models/shape_predictor_68_face_landmarks_GTX.dat") else {
        panic!("Unable to load landmark predictor!");
    };

    let Ok(face_encoder_network) = FaceEncoderNetwork::open("./starter_template/models/dlib_face_recognition_resnet_model_v1.dat") else {
        panic!("Unable to load face encoder network!");
    };

    struct Person<'classification> {
        name: &'classification str,
        encoding: FaceEncoding
    }

    let files = vec!["hillary_1.jpg", "obama_1.jpg", "obama_2.jpg"];

    let mut dataset: Vec<Person> = vec![];

    for file in files {
        let found_encoding = get_encoding(format!("./starter_template/assets/{file}").as_str(), &cnn_detector, &landmark_predictor, &face_encoder_network);

        dataset.push(
            Person {
                name: file,
                encoding: found_encoding
            }
        );
    }

    for base_person in dataset.iter() {
        for comparison_person in dataset.iter() {
            let calculated_distance = base_person.encoding.distance(&comparison_person.encoding);
            println!("{} vs {}: {}", base_person.name, comparison_person.name, calculated_distance);
        }
    }
}
