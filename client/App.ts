import {
  h,
  Component,
} from "https://cdn.jsdelivr.net/npm/preact/dist/preact.mjs";
import { type Todo, list, update, add } from "./api.js";

type State = {
  todos: Todo[];
  sending: boolean;
};

export class App extends Component<{}, State> {
  constructor() {
    super();
    this.state = {
      todos: [],
      sending: false,
    };
  }

  getData() {
    return list().then((todos) =>
      this.setState({
        todos,
      })
    );
  }

  componentDidMount() {
    return this.getData();
  }

  send(e: h.JSX.TargetedEvent) {
    this.setState({ ...this.state, sending: true });
    e.preventDefault();
    const form = e.target as HTMLFormElement;
    const formData = new FormData(form);
    const title = formData.get("title") as string;
    return add({ title }).then((todo) => {
      form.reset();
      return this.setState((s) => ({
        todos: [...s.todos, todo],
        sending: false,
      }));
    });
  }

  complete(todo: Todo) {
    return update(todo.id, { ...todo, completed: !todo.completed }).then(
      (todo) =>
        this.setState((s) => ({
          ...s,
          todos: s.todos.map((t) => {
            if (t.id === todo.id) return todo;
            else return t;
          }),
        }))
    );
  }

  render() {
    return h(
      "div",
      {},
      h("h1", {}, "TODO List"),
      h(
        "form",
        {
          onSubmit: (e) => this.send(e),
        },
        h("input", {
          type: "text",
          id: "title",
          placeholder: "What do you have to do",
          name: "title",
          required: true,
        }),
        h("button", { disabled: this.state.sending }, "Add")
      ),
      h(
        "ul",
        {},
        this.state.todos.map((todo) =>
          h(
            "li",
            { onClick: () => this.complete(todo) },
            h("span", {}, todo.completed ? "✅" : ""),
            h("span", {}, todo.title)
          )
        )
      )
    );
  }
}
