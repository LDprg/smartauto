import { LitElement, html, unsafeCSS } from 'lit';
import { customElement, property } from 'lit/decorators.js';

import { createClient } from '@connectrpc/connect';
import { createGrpcWebTransport } from '@connectrpc/connect-web';

import { GreeterService, SayHelloRequestSchema } from './gen/helloworld/v1/helloworld_pb';
import { create } from '@bufbuild/protobuf';

import globalStyles from './index.css?inline';

@customElement('my-element')
export class MyElement extends LitElement {
  @property()
  docsHint = 'Click on the Vite and Lit logos to learn more';

  @property()
  grpc = 'Loading...';

  @property({ type: Number })
  count = 0;

  constructor() {
    super();

    // this.callSayHello();
  }

  private async callSayHello() {
    const transport = createGrpcWebTransport({
      baseUrl: 'http://127.0.0.1:3000',
    });
    const client = createClient(GreeterService, transport);

    const message = create(SayHelloRequestSchema, {
      name: 'web123',
    });

    console.log('Message: ' + message.name);

    const resp = await client.sayHello(message);

    this.grpc = resp.message;

    console.log(resp.message);
  }

  render() {
    return html`
      <div>
        <button @click=${this._onClick} part="button">count is ${this.count}</button>
      </div>
      <p class="bg-purple-950">${this.grpc}</p>
      <p class="text-3xl font-bold underline">${this.docsHint}</p>
      <div class="bg-purple-500 font-bold">Tailwind</div>
    `;
  }

  private _onClick() {
    this.count++;
  }

  static styles = [unsafeCSS(globalStyles)];
}

declare global {
  interface HTMLElementTagNameMap {
    'my-element': MyElement;
  }
}
