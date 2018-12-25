(function() {
  const codeSegment = document.getElementById('diagonal-code-segment')
  const writingSegment = document.getElementById('diagonal-writing-segment')

  const codeSegmentMaxLines = 30
  const codeSegmentTop = [3, 5]            // Start drawing code from x=3, y=5
  const codeSegmentIdentLineDelta = [6, 3] // Add x+5, y+3 to indent the next line of code

  // Produce an HTML <text> element positioned at the given coordinates.
  const text = (x, y, className, text) => {
    let newNode = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    newNode.setAttribute('x', `${x}`)
    newNode.setAttribute('y', `${y}`)
    newNode.textContent = text

    newNode.classList.add(className)

    return newNode
  }

  // Produce an HTML <text> element vertically aligned and trimmed
  // to be displayed in the code segment.
  const code = (line, indent, code) => {
    const startX = indent * codeSegmentIdentLineDelta[0] + codeSegmentTop[0]
    const startY = line * codeSegmentIdentLineDelta[1] + codeSegmentTop[1]

    const charLimit = 80 - (3.15 * line) - (4 * indent)

    return text(startX, startY, 'code', code.slice(0, charLimit))
  }

  // Construct a representation of a state machine that can be operated on
  // to animate text scrolling through the code segment.
  const codeAnimator = (language, source) => ({
    _startLine: 0,
    _linesToShow: 0,
    _language: language,
    _source: source,
    _animationRate: 0.35,
  })

  const _animatorStep = animator => {
    const codeToText = (state, line) => {
      state.nodes.push(code(state.lineNumber, line.indent, line.code))
      return {
        nodes: state.nodes,
        lineNumber: state.lineNumber + 1,
      }
    }
    const linesToShow = Math.min(codeSegmentMaxLines, animator._linesToShow + 1)
    const source = animator._source.linesOfCode.slice(
      animator._startLine,
      animator._startLine + linesToShow)

    if (linesToShow === animator._linesToShow) {
      animator._startLine += 1
    }
    animator._linesToShow = linesToShow
    
    let init = {
      nodes: [],
      lineNumber: 0,
    }

    const { nodes, lineNumber } = FN.reduce(codeToText, source, init)
    return {
      animator,
      nodes,
    }
  }

  const language = CODE_SNIPPETS[0].languageName
  const sources = CODE_SNIPPETS[0].sourceFiles
  let animator = codeAnimator(language, sources[0])

  let i = 0
  for (i = 0; i < 5; i++) {
    setTimeout(() => {
      let { animator: anim , nodes } = _animatorStep(animator)
      animator = anim
      console.log(animator)
      console.log(nodes)
      for (const node of nodes) {
        codeSegment.appendChild(node)
      }
      setTimeout(() => {
        let txts = document.getElementsByTagName('text')
        let i = 0
        for (i = 0; i < txts.length; i++) {
          txts[i].remove()
        }
      }, 100)
    }, i * 1000)
  }
})()
