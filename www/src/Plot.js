import React from 'react'

const Plot = ({ width, height, config }) => {
  const imageIds = [1]
  const [images, setImages] = React.useState([])

  React.useEffect(() => {
    async function getImage(_) {
      let imageBlob
      try {
        var res = await fetch('http://localhost:5000/plot', {
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

  return <img
    src={images[0]}
    className="app-canvas"
    width={width}
    height={height}
  />
}

export default Plot
