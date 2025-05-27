(async () => {
  const
    response = await fetch('/params.bin'),
    reader = response.body!.getReader(),
    data: Uint8Array[] = [],
    modelBytesTotal = parseInt(response.headers.get('content-length')!)

  let modelBytesReceived = 0

  postMessage({
    type: 'loadStart',
    modelBytesTotal,
  })

  while (true) {
    const { done, value } = await reader.read()

    if (done) {
      break
    }

    modelBytesReceived += value.byteLength

    postMessage({
      type: 'loadProgress',
      modelBytesReceived,
    })

    data.push(value)
  }

  const
    buffer = await new Blob(data).arrayBuffer(),
    dataview = new DataView(buffer),
    numberOfLayers = Number(dataview.getBigUint64(0, true)),
    structure = Array.from({ length: numberOfLayers }, (_, i) =>
      Number(dataview.getBigUint64(8 * (i + 1), true))
    )

  let byteOffset = 8 * numberOfLayers

  const getNextParam = () => {
    return dataview.getFloat64(byteOffset += 8, true)
  }

  const model = Array.from({
    length: numberOfLayers - 1,
  }, (_, layerIndex) =>
    Array.from({ length: structure[layerIndex + 1] }, () => ({
      weights: Array.from({ length: structure[layerIndex] }, () =>
        getNextParam()
      ),
      bias: getNextParam(),
    }))
  )

  postMessage({
    type: 'loadEnd',
    model,
  })
})()
