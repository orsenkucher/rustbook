import React from 'react'

const Plot = ({ width, height, config }) => {
  const imageIds = [0, 1, 2, 3]
  const [images, setImages] = React.useState([])
  const [current, setCurrent] = React.useState(-1)

  React.useEffect(() => {
    async function getImage(i) {
      let imageBlob
      try {
        var res = await fetch(`http://localhost:5000/plot/${i}`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          // body: JSON.stringify(config)
          body: config
        });

        imageBlob = await res.blob();
      } catch (err) {
        return null
      }
      return URL.createObjectURL(imageBlob)
    }
    async function getImages() {
      const imageArray = []
      for (const id of imageIds) {
        imageArray.push(await getImage(id))
      }
      setImages(imageArray)
    }

    getImages()
  }, [config])

  const draw = (i) => <img
    src={images[i]}
    className="app-canvas"
    width={width / 2}
    height={height / 2}
    onClick={() => {
      console.log("Current", i)
      setCurrent(i)
    }}
  />

  if (current < 0) {
    return (
      <div>
        <div className='app-row'>
          {draw(0)}
          {draw(1)}
        </div>
        <div className='app-row'>
          {draw(2)}
          {draw(3)}
        </div>
      </div>
    )
  }

  return (
    <div>
      <button className='back-button' onClick={() => setCurrent(-1)}>Back</button>
      <img
        src={images[current]}
        className="app-canvas"
        width={width}
        height={height}
      />
    </div>
  )
}

export default Plot
