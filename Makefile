gui-windows:
	cargo build --target x86_64-pc-windows-gnu --package dupels-gui

gui-windows/release:
	cargo build --target x86_64-pc-windows-gnu --package dupels-gui --release