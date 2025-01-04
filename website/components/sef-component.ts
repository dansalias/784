export class SefComponent extends HTMLElement {
  constructor(componentName: string) {
    super()

    const template = document.querySelector(
      `template#${componentName}`
    ) as HTMLTemplateElement

    if (template) {
      this.appendChild(template.content.cloneNode(true))
    }
  }
}
