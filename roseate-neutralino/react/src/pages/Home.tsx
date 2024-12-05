import React from 'react'

import '../styles/pages/home.scss'

// Components
import * as Fa6 from 'react-icons/fa6'

export default function Home(): React.ReactElement {
	return (
		<div className='page home'>
			<h1>Welcome Home</h1>
			<Fa6.FaReact size={50} color='#61dafb' />
			<p>This is the homepage.</p>
		</div>
	)
}
