import React, { useState, useEffect } from 'react';

export default function RawBack() {
  const [backResp, setBackResp] = useState("blop");

  useEffect(() => {
    fetch(
      `${process.env.REACT_APP_RASPBERRY || ""}/`,
      {
        method: "GET",
      }
    )
      .then(result => result.text())
      .then(result => {
        return setBackResp(result);
      })
  }, []);
  return <div> <b> {backResp} </b> </div>
}