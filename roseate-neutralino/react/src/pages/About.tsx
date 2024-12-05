import React from 'react'
import axios from 'axios'

import '../styles/pages/about.scss'

// Components
import { CodeBlock, dracula } from 'react-code-blocks';

export default function About(): React.ReactElement {
	const [data, setData] = React.useState(null)

	React.useEffect(() => {
		axios.get('https://jsonplaceholder.typicode.com/posts/1')
			.then(response => setData(response.data))
			.catch(error => console.error(error))
	}, [])

	return (
		<div className='page about'>
			<h1>About Page</h1>
			{data
				? <CodeBlock text={JSON.stringify(data, null, 2)} language='json' theme={dracula} />
				: <p>Loading...</p>}
		</div>
	)
}
