import styles from "./styles.module.css";

type Props = React.FormHTMLAttributes<HTMLFormElement> & {
  "data-current-item"?: "connecting" | "claiming";
};

export default function FormMain({ children, ...props }: Props) {
  return (
    <form className={styles.form} {...props}>
      {children}
    </form>
  );
}
