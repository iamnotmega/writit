<div align="center">
  <img src="https://github.com/user-attachments/assets/5ea1e7a8-b09d-4b80-a077-3a7764b2f01c" width=1920 height=1080 alt="Writit preview"/>

  <br/>
  <br/>

  # Writit
  **Fast, free and open source notes**
</div>

<br/>
<br/>

Writit is a free and open-source notes app written in Rust using [Tauri](https://v2.tauri.app) and [SvelteKit](https://svelte.dev/docs/kit/introduction).

<br/>

## Why Writit?
- **Blazingly fast**: Thanks to the incredible performance of Writit's tech stack, you can write notes faster than ever before.
- **Free & open source**: Writit is completely free of charge and its source code can be read by anyone.
- **Tiny footprint**: Writit consumes very little RAM while in use and takes up little to no space.
- **Distraction-free**: Say goodbye to unwanted AI integration and features. Writit is simple by design.

## Installation
### Windows
> [!WARNING]
> Writit does **not support 32-bit** machines or machines running on the ARM architecture.

Download the `.exe` file from the [latest release.](https://github.com/iamnotmega/writit/releases/latest) Once you have downloaded it, open the file. You should see a setup window pop up. Follow the prompts to install Writit on your machine.

### macOS
> [!WARNING]
> Writit requires an **Apple Silicon** Mac (M1 or newer). Intel-based Macs **are not supported**.

Download the `.dmg` file from the [latest release.](https://github.com/iamnotmega/writit/releases/latest) Once you have downloaded it, open the file. You should see a window with the Writit icon and your Applications folder. To install Writit, drag the icon into the folder. macOS will then copy the contents of the `.dmg` file into the Applications folder. After it has installed, you can now launch the app.

### Linux
Download the raw Linux binary from the [latest release.](https://github.com/iamnotmega/writit/releases/latest) Once downloaded, you have to make it executable:

**Method 1:**
- Open your file manager (e.g Nautilus on GNOME, Dolphin on KDE)
- Find the downloaded binary
- Right-click, open properties and look for an "Executable as program" option or similar
- Make sure it is enabled

Once it has been enabled and the binary is executable, double click on it to launch it.

**Method 2:**
If you are using a terminal, you can make the binary executable like so:
```bash
chmod +x <binary>
```
You can run it with:
```bash
./<binary>
```

Replace `<binary>` with the location of your downloaded binary.

### Compiling from source
> [!NOTE]
> Make sure you have **Node.js** and **Rust** installed on your machine before compiling from source.

If you wish, you can also compile Writit from source:

1. Clone the repository to your local machine:
```bash
git clone https://github.com/iamnotmega/writit.git
```

2. Change your current directory to the cloned repository:
```bash
cd writit
```

3. Install the frontend dependencies:
```bash
npm install
```

4. Build the binary for your operating system:
```bash
npm run tauri build
```
Your built binary will be located in `src-tauri/target/release` or `src-tauri/target/release/bundles/` depending on your operating system. You can then run your compiled binary by following the platform-specific instructions for your operating system listed above.

Alternatively, if you wish to run the app immediately after its built, run `npm run tauri dev` instead. Please note that this is **not meant for production** and should be used only by developers.

## License
Writit is licensed under the [GPL-3.0](LICENSE) license.
