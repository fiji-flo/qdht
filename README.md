# Quick and Dirty Helm Template

The goal of this project is to have a simple single binary tool to render helm like charts.

Given the following directory tree:

```
chart
├── templates
│   ├── 00-namespace.yaml
│   ├── deployment.yaml
│   └── service.yaml
├── values
│   ├── dev.yaml
│   └── prod.yaml
└── values.yaml
```

Running:

```
qdht -f chart/values.yaml -f chart/values/dev.yaml chart
```

will out put all templates rendered with the values from `values.yaml` merged with `values/dev.yaml`.
The values from the last value file will be preferred.