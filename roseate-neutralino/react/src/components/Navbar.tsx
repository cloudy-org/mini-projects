import React from 'react'

import '../styles/components/navbar.scss'

// Components
import { Link } from 'react-router-dom'

export default function Navbar(): React.ReactElement {
	return (
		<nav className='navbar'>
			<Link to='/'>Home</Link>
			<Link to='/about'>About</Link>
		</nav>
	)
}
