import torch
import torch.nn.functional as F
import torch.optim as optim
from torch.utils.data import DataLoader
from torchvision import datasets, transforms
import json

def train(epochs=2, batch_size=64, lr=0.01):
    transform = transforms.ToTensor()
    train_dataset = datasets.MNIST(root="data", train=True, download=True, transform=transform)
    train_loader = DataLoader(train_dataset, batch_size=batch_size, shuffle=True)

    conv_w = torch.randn(8, 1, 3, 3, requires_grad=True)
    conv_b = torch.zeros(8, requires_grad=True)
    fc_w = torch.randn(10, 8 * 26 * 26, requires_grad=True)
    fc_b = torch.zeros(10, requires_grad=True)

    optimizer = optim.SGD([conv_w, conv_b, fc_w, fc_b], lr=lr)

    for _ in range(epochs):
        for data, target in train_loader:
            optimizer.zero_grad()
            x = F.conv2d(data, conv_w, conv_b)
            x = F.relu(x)
            x = torch.flatten(x, 1)
            x = F.linear(x, fc_w, fc_b)
            loss = F.cross_entropy(x, target)
            loss.backward()
            optimizer.step()

    trained_params = {
        "conv_weight": conv_w.detach().cpu().numpy().tolist(),
        "conv_bias": conv_b.detach().cpu().numpy().tolist(),
        "fc_weight": fc_w.detach().cpu().numpy().tolist(),
        "fc_bias": fc_b.detach().cpu().numpy().tolist(),
    }

    return json.dumps(trained_params)

