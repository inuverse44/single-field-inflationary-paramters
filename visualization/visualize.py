import pandas as pd
import matplotlib.pyplot as plt
import glob
import os

def plot_planck_bk18_observation(ax, planck_2sigma_file, planck_1sigma_file): 
    try:
        # Read and plot 2-sigma contour (light gray, background)
        df_2sigma = pd.read_csv(planck_2sigma_file, sep='\t', header=None, names=['ns', 'r'], engine='python')
        ax.fill(df_2sigma['ns'], df_2sigma['r'], alpha=0.5, color='lightgray', label='Planck 2018 $2\sigma$')

        # Read and plot 1-sigma contour (darker gray, foreground)
        df_1sigma = pd.read_csv(planck_1sigma_file, sep='\t', header=None, names=['ns', 'r'], engine='python')
        ax.fill(df_1sigma['ns'], df_1sigma['r'], alpha=0.5, color='gray', label='Planck 2018 $1\sigma$')
        
    except FileNotFoundError as e:
        print(f"Observational data file not found: {e}. Skipping.")
    except Exception as e:
        print(f"Could not process observational data: {e}")

def plot_ns_r(ax, csv_files, plot_filename):
    if not csv_files:
        print("No .csv files found in 'output'. Skipping simulation data plot.")
    else:
        for file_path in csv_files:
            try:
                df = pd.read_csv(file_path)
                if not df.empty and 'ns' in df.columns and 'r' in df.columns:
                    sim_name = df['simulation_name'].iloc[0]
                    param_name = df['parameter_name'].iloc[0]
                    label = f'{sim_name} ({param_name} scan)'
                    ax.plot(df['ns'], df['r'], marker='.', linestyle='-', label=label)
            except Exception as e:
                print(f"Could not process file {file_path}: {e}")

    # --- 3. Finalize Plot ---
    ax.set_xlabel('Spectral Index $n_s$', fontsize=14)
    ax.set_ylabel('Tensor-to-Scalar Ratio $r$', fontsize=14)
    ax.set_title('$n_s$ vs $r$ Diagram', fontsize=16)
    ax.set_yscale('log')
    ax.legend(fontsize=12)
    ax.grid(True, which="both", ls="--", c='0.7')
    plt.tight_layout()

    try:
        plt.savefig(plot_filename, dpi=300)
        print(f"Successfully saved plot to: {plot_filename}")
    except Exception as e:
        print(f"Failed to save plot: {e}")


def create_ns_r_diagram():
    """
    Scans the 'output' directory for .csv files, plots the ns-r diagram
    for each simulation, and overlays Planck 2018 observational data.
    """
    output_dir = 'output'
    observation_dir = 'observation'
    plot_filename = os.path.join(output_dir, 'ns_r_diagram.png')
    csv_files = glob.glob(os.path.join(output_dir, '*.csv'))

    plt.style.use('seaborn-v0_8-whitegrid')
    fig, ax = plt.subplots(figsize=(12, 8))

    # --- 1. Plot Observational Data (Planck 2018) ---
    planck_1sigma_file = os.path.join(observation_dir, 'Planck_BK18_ns_r_1sigma.txt')
    planck_2sigma_file = os.path.join(observation_dir, 'Planck_BK18_ns_r_2sigma.txt')
    plot_planck_bk18_observation(ax, planck_2sigma_file, planck_1sigma_file)
    
    # --- 2. Plot Simulation Data ---
    plot_ns_r(ax, csv_files, plot_filename)

if __name__ == '__main__':
    create_ns_r_diagram()