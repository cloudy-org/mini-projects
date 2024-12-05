# Mini-Projects Repository

Welcome to the **Mini-Projects** repository! This repository contains various mini-projects developed by multiple contributors. The current branch, `roseate-neutralino`, focuses on recreating the "roseate" app, a fast image viewer originally written in Rust, using TypeScript, Neutralino, React, Vite, and TypeScript.

## Project: [Roseate-Neutralino](/roseate-neutralino)

### Overview

The goal of this project is to recreate the "roseate" image viewer application using modern web technologies. The original "roseate" app is known for its speed and efficiency, and we aim to achieve similar performance with our implementation.

### Technologies Used

- **Neutralino**: A lightweight and portable application development framework.
- **React**: A JavaScript library for building user interfaces.
- **Vite**: A fast build tool and development server.
- **TypeScript**: A typed superset of JavaScript that compiles to plain JavaScript.
- **SCSS**: A CSS preprocessor that adds power and elegance to the basic language.

### Getting Started

To get started with the project, follow these steps:

1. **Clone the repository**:
	```sh
	git clone -b roseate-neutralino https://github.com/cloudy-org/mini-projects.git
	cd mini-projects
	```

2. **Install dependencies**:
	```sh
	npm install
	```

3. **Run the development server**:
	```sh
	npm run dev
	```

4. **Build the project**:
	```sh
	npm run build
	```

### Project Structure

The project structure is organized as follows:

```
mini-projects/
├── roseate-neutralino/				# Roseate Neutralino project
    ├── react/              		# Source code for the React app
    │   ├── public/          		# Public assets
    │   ├── src/             		# Source code
    │   │   ├── components/  		# React components
    │   │   ├── pages/       		# React pages
    │   │   ├── styles/      		# SCSS styles
    │   │   ├── App.tsx      		# Main React component
    │   │   ├── main.tsx	   		# Entry point
    │   ├── index.html        		# HTML template
    │   ├── package.json      		# NPM package file
    │   ├── tsconfig.json     		# TypeScript configuration file
    │   ├── vite.config.ts    		# Vite configuration file
    ├── neutralino.config.json		# Neutralino configuration file
```

### Contributing

We welcome contributions from everyone. To contribute to this project, please follow these steps:

1. **Fork the repository**.
2. **Create a new branch**:
	```sh
	git checkout -b feature/your-feature-name
	```
3. **Make your changes**.
4. **Commit your changes**:
	```sh
	git commit -m "Add your commit message"
	```
5. **Push to the branch**:
	```sh
	git push origin feature/your-feature-name
	```
6. **Create a pull request**.

---

Happy coding!
