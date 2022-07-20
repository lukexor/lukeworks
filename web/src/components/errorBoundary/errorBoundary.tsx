import Link from "next/link";
import { NextRouter, withRouter } from "next/router";
import { Component, ErrorInfo, ReactNode } from "react";
import s from "./errorBoundary.module.css";

export type Props = {
  router: NextRouter;
  children?: ReactNode;
};

type State = {
  error: null | Error;
};

export default withRouter(
  class ErrorBoundary extends Component<Props, State> {
    public state: State;

    constructor(props: Props) {
      super(props);
      this.state = { error: null };
    }

    public static getDerivedStateFromError(error: Error): State {
      return { error };
    }

    public componentDidMount(): void {
      this.props.router.beforePopState(() => {
        this.setState({ error: null });
        return true;
      });
    }

    public componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
      console.error(error, errorInfo);
    }

    public render(): ReactNode {
      if (this.state.error) {
        return (
          <section className={s.wrapper}>
            <h1>Oops, something has gone horribly wrong.</h1>
            <Link className={s.backButton} href={document.referrer}>
              <a
                onClick={() => {
                  this.setState({ error: null });
                  this.props.router.back();
                  return false;
                }}
              >
                Go back
              </a>
            </Link>
          </section>
        );
      }
      return this.props.children;
    }
  },
);
