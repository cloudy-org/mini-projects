import React from 'react'
import { filesystem } from '@neutralinojs/lib'

export default function App(): React.ReactElement {
	React.useEffect(() => {
		filesystem.readDirectory('./')
			.then((data) => console.log(data))
			.catch((error) => console.error(error))
	}, [])

	return (
		<div>
			<h1>Hello, World!</h1>
			<h5>Built with Vite + React + TypeScript</h5>
			<button onClick={() => window.alert('Hello, Neutralino!')}>Click me!</button>
		</div>
	)
}
