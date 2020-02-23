public class Test {
    public static native Object ser(int n);
    public static native void de(Object obj);
    public static native Object roundtrip(Object obj);

    static {
        System.loadLibrary("testlib");
    }
}
