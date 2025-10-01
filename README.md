## Chimera Modulation Demo

This repository contains a functional implementation of the "Raman Whisper" modulation pipeline along with a companion Jupyter notebook for interactive exploration. The original monolithic Colab script has been refactored into a reusable Python package so that you can import the core logic in your own projects or run end-to-end simulations programmatically.

### Package layout

```
chimera/
├── chimera/                # Library package
│   ├── __init__.py
│   ├── config.py           # Dataclass-based configuration objects
│   ├── pipeline.py         # Encode → transmit → decode pipeline
│   └── utils.py            # Bitstream helpers and logging utilities
├── tests/                  # Light-weight smoke tests
│   └── test_utils.py
└── chimera.ipynb           # Interactive walkthrough using the library
```

### Getting started

1. Install the required runtime dependencies (preferably inside a virtual environment):

	```bash
	pip install -r requirements.txt
	```

2. Run the utility smoke tests:

	```bash
	pytest
	```

3. Launch Jupyter Notebook or VS Code's notebook interface and open `chimera.ipynb` to step through the pipeline.

### Using the library

```python
from chimera import run_simulation, SimulationConfig, ProtocolConfig

sim_config = SimulationConfig(sample_rate=32_000, snr_db=5.0)
protocol = ProtocolConfig()

result = run_simulation(sim_config=sim_config, protocol=protocol, verbose=True)
print(result.demodulation.recovered_message)
```

### License

This project inherits the license from the original repository.
