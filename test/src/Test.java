public class Test {
    public static native Object ser(int n);
    public static native void de(Object obj);
    public static native Object roundtrip(Object obj);

    public static native Object canadaSer();
    public static native void canadaDe(Object obj);

    public static native String canadaSerJson();

    public static native Object twitterSer();
    public static native void twitterDe(Object obj);


    static {
        System.loadLibrary("testlib");
    }
}
