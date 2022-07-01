import { useState } from 'react';
import './App.css';
import AttributesTest from './AttributesTest';

function App() {


let db: IDBDatabase | null = null;
let [dbLog] = useState<string[]>([]);

if( window && window.indexedDB && !db ) {

  dbLog.push('window.indexedDB exists' );
  let DBOpenRequest = window.indexedDB.open("savaged");

  DBOpenRequest.onerror = function(event) {
    console.log("Error?")
    dbLog.push('Error loading database' );
  };

  DBOpenRequest.onsuccess = function(event) {
    // console.log("Success?")
    dbLog.push('Database initialized.');

    // store the result of opening the database.
    db = DBOpenRequest.result;
    db.onerror = function(event) {
      // Generic error handler for all errors targeted at this database's requests!
      if( event ) {
        // @ts-ignore
        console.error("Database error: " + (event && event.target && event.target.errorCode ? event.target.errorCode : "No event" )) ;
      }
    };
  };

} else {
  dbLog.push("Your browser doesn't support a stable version of IndexedDB. Such and such feature will not be available.");
}

  return (
    <div className="App" style={{textAlign: "left", padding: "3rem"}}>

        <AttributesTest />

    <hr />
    <div style={{textAlign: "center"}}>
      Code Repo at <a href="https://github.com/jdgwf/savaged-rust-wasm-test">https://github.com/jdgwf/savaged-rust-wasm-test</a>
    </div>
    </div>
  );


}
export default App;
