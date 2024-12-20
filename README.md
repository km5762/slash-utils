# /UTILS - Cryptography Education

/UTILS is an educational website designed to help students understand complex cryptographic algorithms by providing a simplified, interactive, and lightweight interface. The website focuses on common cryptographic algorithms, including ECDSA, ECDH, AES, and more.

## Live Demo

You can explore the project live at the following URL:

[Live Demo - Cryptography](https://slash-utils.pages.dev/)

## Features

- **Cryptographic Algorithms**: Simplified explanations and visualizations of cryptographic algorithms, including ECDSA, ECDH, and AES.
- **Custom Implementations**: All cryptographic algorithms are implemented entirely from scratch in Rust with WebAssembly, reducing dependency overhead and optimizing performance.
- **Frontend**: Built with VueJS, Tailwind CSS, and Pinia, with static site generation powered by AstroJS.
- **Optimized for Performance**: Algorithms are optimized for lightweight bundles, ensuring fast load times for an educational experience.

## Technologies Used

- **Frontend**: VueJS, Tailwind CSS, Pinia, AstroJS
- **Backend**: Rust, WebAssembly (WASM)
- **Cryptographic Algorithms**: ECDSA, ECDH, AES, SHA256 (including custom big number, modular arithmetic, elliptic curve, and hashing implementations)

## Installation

To run the project locally, follow these steps:

1. Clone the repository:

   ```bash
   git clone git@github.com:km5762/slash-utils.git
   ```

2. Navigate to the project directory:

   ```bash
   cd slash-utils
   ```

3. Install dependencies:

   ```bash
   npm install
   ```

4. Run the project locally:

   ```bash
   npm run dev
   ```

## Contributing

Feel free to open issues or submit pull requests for any improvements, bug fixes, or suggestions.
