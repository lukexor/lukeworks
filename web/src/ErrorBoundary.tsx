import { Component, ErrorInfo, ReactNode } from "react";

type Props = {
  children: ReactNode;
};

type State = {
  error: null | Error;
};

// TODO: ErrorBoundary
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
      return <h1>Oops, something has gone horribly wrong.</h1>;
    }
    return this.props.children;
  }
}

export default ErrorBoundary;
