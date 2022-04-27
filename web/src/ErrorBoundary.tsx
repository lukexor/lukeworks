import "./ErrorBoundary.css";
import { Component, ErrorInfo, ReactNode } from "react";
import { Link, NavigateFunction } from "react-router-dom";

type Props = {
  navigate: NavigateFunction;
  children: ReactNode;
};

type State = {
  error: null | Error;
};

class ErrorBoundary extends Component<Props, State> {
  public state: State;

  constructor(props: Props) {
    super(props);
    this.state = { error: null };
  }

  public static getDerivedStateFromError(error: Error): State {
    return { error };
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
    console.error(error, errorInfo);
  }

  public render(): ReactNode {
    if (this.state.error) {
      return (
        <section className="error-boundary">
          <h1>Oops, something has gone horribly wrong.</h1>
          <Link
            className="button"
            to={document.referrer}
            onClick={() => {
              this.setState({ error: null });
              this.props.navigate(-1);
              return false;
            }}
          >
            Go back
          </Link>
        </section>
      );
    }
    return this.props.children;
  }
}

export default ErrorBoundary;
