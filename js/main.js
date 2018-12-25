(function() {
  const text = (x, y, classes, text) => {
    let newNode = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    newNode.setAttribute('x', `${x}`)
    newNode.setAttribute('y', `${y}`)
    newNode.textContent = text

    for (const className of classes) {
      newNode.classList.add(className)
    }

    return newNode
  }

  const codeSegment = document.getElementById('diagonal-code-segment')
  const writingSegment = document.getElementById('diagonal-writing-segment')

  const codeSegmentTop = [3, 5]            // Start drawing code from x=3, y=5
  const codeSegmentIdentLineDelta = [5, 3] // Add x+5, y+3 to indent the next line of code

  codeSegment.appendChild(text(3, 5, ['code', 'red'], 'def test():'))
  codeSegment.appendChild(text(9, 8, ['code', 'blue'], 'print("Hello, world")'))
})()
