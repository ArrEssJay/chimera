"""Main script to run the Chimera simulation."""
from chimera.pipeline import run_simulation
from chimera.config import SimulationConfig

if __name__ == "__main__":
    # Configure the simulation to generate plots
    sim_config = SimulationConfig(generate_plots=True)

    # Run the end-to-end simulation
    result = run_simulation(sim_config=sim_config, verbose=True)

    # Print the final results
    print("\n--- SIMULATION COMPLETE ---")
    print(f"Recovered Message: '{result.demodulation.recovered_message}'")
    print(f"Post-FEC BER: {result.demodulation.post_fec_ber:.6f}")
