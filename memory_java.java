class Data {
    int value = 10;
}

public class Example {
    public static void main(String[] args) {
        Data first = new Data();
        Data second = first; // Both reference the same object.

        first = null;
        System.out.println(second.value); // Still accessible.

        second = null; // No references remain in this example.
    }
}
