import React from 'react'
import ReactDOM from 'react-dom/client'
import { init } from '@neutralinojs/lib'

import './styles/main.scss'

// Components
import App from './App'

const container = document.getElementById('root')
if (!container) throw new Error('No root element found')

const root = ReactDOM.createRoot(container)
root.render(<App />)

init()
void React
