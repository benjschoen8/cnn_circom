import torch
import torch.nn as nn
import torch.nn.functional as F
from torchvision import datasets, transforms
from torch.utils.data import DataLoader
import json

class SimpleCNN(nn.Module):
    def __init__(self):
        super(SimpleCNN, self).__init__()
        self.conv1 = nn.Conv2d(1, 8, kernel_size=3, padding=1)
        self.relu1 = nn.ReLU()
        self.pool1 = nn.MaxPool2d(2)

        self.conv2 = nn.Conv2d(8, 16, kernel_size=3, padding=1)
        self.relu2 = nn.ReLU()
        self.pool2 = nn.MaxPool2d(2)

        self.flatten = nn.Flatten()
        self.fc = nn.Linear(16 * 7 * 7, 10)

    def forward(self, x):
        x = self.relu1(self.conv1(x))
        x = self.pool1(x)
        x = self.relu2(self.conv2(x))
        x = self.pool2(x)
        x = self.flatten(x)
        x = self.fc(x)
        return x

def train(epochs=1, batch_size=64, lr=0.01):
    transform = transforms.ToTensor()
    train_dataset = datasets.MNIST(root="data", train=True, download=True, transform=transform)
    train_loader = DataLoader(train_dataset, batch_size=batch_size, shuffle=True)

    model = SimpleCNN()
    optimizer = torch.optim.SGD(model.parameters(), lr=lr)

    for _ in range(epochs):
        for data, target in train_loader:
            optimizer.zero_grad()
            output = model(data)
            loss = F.cross_entropy(output, target)
            loss.backward()
            optimizer.step()

    # 組裝結構資訊成 dict
    model_structure = []
    for name, module in model.named_modules():
        if name == "":
            continue  # skip top-level model itself
        layer_info = {
            "layer_name": name,
            "layer_type": module.__class__.__name__,
        }
        params = []
        for param_name, param in module.named_parameters(recurse=False):
            params.append({
                "param_name": param_name,
                "shape": tuple(param.shape),
                "values": param.detach().cpu().numpy().tolist()
            })
        if params:
            layer_info["parameters"] = params
        else:
            layer_info["parameters"] = None
        model_structure.append(layer_info)

    return json.dumps(model_structure, indent=4)

if __name__ == "__main__":
    trained_json = train()
    print(trained_json)
