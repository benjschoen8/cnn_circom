# dense_test.py

import numpy as np

def dense_layer(input_vector, weight_matrix, bias_vector):
    input_vector = np.array(input_vector)
    weight_matrix = np.array(weight_matrix)
    bias_vector = np.array(bias_vector)
    
    return np.dot(weight_matrix, input_vector) + bias_vector

if __name__ == "__main__":
    input_vector = [1.0, 2.0, 3.0]
    weight_matrix = [
        [0.2, 0.8, -0.5],
        [0.5, -0.91, 0.26]
    ]
    bias_vector = [2.0, 3.0]

    result = dense_layer(input_vector, weight_matrix, bias_vector)
    print("Python Dense Layer Output:", result)

