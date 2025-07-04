mod framework;
mod adapter;
mod usecase;
mod entity;

use crate::framework::pytorch::Pytorch;
use crate::adapter::python_embedding::PythonEmbedding;
use crate::adapter::python_trainer::PythonTrainer;
use crate::usecase::trainer::Trainer;
use crate::entity::layer::Layer;
use crate::entity::dense::Dense;
use crate::entity::conv2d::Conv2D;

fn main() {
    // 先用 new() 建立預設物件
    let mut conv = Conv2D::new();

    // 設定欄位
    conv.input_name = "input".to_string();
    conv.name = "conv_test".to_string();
    conv.input_size = (1, 4, 4);        // 1通道，4x4 高寬
    conv.weight_size = (1, 1, 3, 3);    // 1輸出通道，1輸入通道，3x3 kernel

    conv.weight = vec![
        vec![
            vec![
                vec![0.0, 1.0, 0.0],
                vec![1.0, -4.0, 1.0],
                vec![0.0, 1.0, 0.0],
            ]
        ]
    ];

    conv.bias = vec![0.0];
    conv.padding = 1;
    conv.stride = 1;

    // 輸入張量 (input tensor)，格式是 [channel][height][width]
    let input_tensor = vec![
        vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.0, 14.0, 15.0, 16.0],
        ],
    ];

    // 呼叫 calculate_test 計算結果
    let output = conv.calculate_test(&input_tensor);

    println!("Conv2D Output:");
    for (c, channel) in output.iter().enumerate() {
        println!("Output channel {}:", c);
        for row in channel {
            println!("{:?}", row);
        }
    }
}
