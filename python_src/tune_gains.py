"""Script to tune the gains of the demodulator."""
import numpy as np
from chimera.pipeline import run_simulation
from chimera.config import SimulationConfig, ProtocolConfig
from chimera.pipeline import _timing_and_carrier_recovery_impl
import matplotlib.pyplot as plt

def run_gain_test(kp_carrier, ki_carrier, kp_timing, ki_timing):
    """Runs a simulation with the given gain values and returns the BER."""
    sim_config = SimulationConfig(generate_plots=False)
    protocol_config = ProtocolConfig()

    # Monkey-patch the timing and carrier recovery function
    def tuned_recovery(baseband_signal, samples_per_symbol, sample_rate):
        return _timing_and_carrier_recovery_impl(
            baseband_signal,
            samples_per_symbol,
            sample_rate,
            kp_carrier=kp_carrier,
            ki_carrier=ki_carrier,
            kp_timing=kp_timing,
            ki_timing=ki_timing,
        )

    from chimera import pipeline
    pipeline.timing_and_carrier_recovery = tuned_recovery

    try:
        result = run_simulation(sim_config=sim_config, protocol=protocol_config, verbose=False)
        return result.demodulation.post_fec_ber
    except RuntimeError:
        return 1.0

if __name__ == "__main__":
    carrier_gains = np.logspace(-5, -3, 5)
    timing_gains = np.logspace(-4, -2, 5)

    results = np.zeros((len(carrier_gains), len(timing_gains)))

    for i, kp_carrier in enumerate(carrier_gains):
        for j, kp_timing in enumerate(timing_gains):
            ki_carrier = kp_carrier**2 / 4.0
            ki_timing = kp_timing / 10.0
            ber = run_gain_test(kp_carrier, ki_carrier, kp_timing, ki_timing)
            results[i, j] = ber
            print(f"kp_carrier={kp_carrier:.6f}, kp_timing={kp_timing:.6f}, BER={ber:.6f}")

    plt.figure(figsize=(10, 8))
    plt.imshow(results, cmap="viridis", interpolation="nearest")
    plt.colorbar(label="Post-FEC BER")
    plt.xticks(np.arange(len(timing_gains)), [f"{g:.4f}" for g in timing_gains], rotation=45)
    plt.yticks(np.arange(len(carrier_gains)), [f"{g:.6f}" for g in carrier_gains])
    plt.xlabel("Timing Gain (Kp_timing)")
    plt.ylabel("Carrier Gain (Kp_carrier)")
    plt.title("Gain Tuning Results")
    plt.savefig("gain_tuning.png")
    plt.close()

    print("\nGain tuning complete. Results saved to gain_tuning.png")
