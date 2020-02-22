public class Test {
    public static native Object test(int n);

    static {
        System.loadLibrary("testlib");
    }
}
