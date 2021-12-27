import React, { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'

// import logo from './logo.svg'
// import tauriCircles from './tauri.svg'
// import tauriWord from './wordmark.svg'
import './App.css'

function App() {
  const [msgFromRust, setMsgFromRust] = useState('')
  const [inputValue, setInputValue] = useState('')

  const handleHelloWorld = async () => {
    try {
      console.log('hello');

      const response = await invoke('hello_world_test', {
        event: inputValue || 'nope',
      });
      
      await invoke('render_map');

      setMsgFromRust(`${response}`)
      console.log('response ', response)
    } catch (error) {
      console.log('error ', error)
    }
  }
  
  return (
    <div className="App">
      <header className="App-header">
        <div className='component-wrapper'>
          <input
            value={inputValue}
            placeholder='input for rust'
            onChange={(e) => setInputValue(e.target.value)}
          />
          <button onClick={handleHelloWorld}>call rust</button>
          {!!msgFromRust && (
            <p style={{ position: 'absolute' }}>
              response message: {msgFromRust}
            </p>
          )}
        </div>
      </header>
    </div>
  )
}

export default App
