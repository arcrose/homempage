(function() {
  const drawText = (container, x, y, classes, text) => {
    let newNode = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    newNode.setAttribute('x', `${x}`)
    newNode.setAttribute('y', `${y}`)
    newNode.textContent = text

    for (const className of classes) {
      newNode.classList.add(className)
    }

    container.appendChild(newNode)

    return newNode
  }

  const codeSegment = document.getElementById('diagonal-code-segment')
  const writingSegment = document.getElementById('diagonal-writing-segment')

  drawText(codeSegment, 3, 5, ['code', 'red'], 'def test():')
  drawText(codeSegment, 9, 8, ['code', 'blue'], 'print("Hello, world")')
})()
