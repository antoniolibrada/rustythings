import {
  h,
  Component,
  render,
} from "https://cdn.jsdelivr.net/npm/preact/dist/preact.mjs";

class App extends Component {
  constructor() {
    super();
    this.state = { todos: [] };
  }
  componentDidMount() {
    fetch("/todo/")
      .then((res) => res.json())
      .then((data) => this.setState({ todos: data }));
  }

  render() {
    return h(
      "ul",
      {},
      this.state.todos.map((todo) => h("li", {}, todo.title))
    );
  }
}

render(h(App), document.body, document.body.lastChild);
