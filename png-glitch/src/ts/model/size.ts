export class Size {
    width = 1920;
    height = 1080;

    static create(videoTrack: MediaStreamVideoTrack): Size {
        const size = new Size();
        const settings = videoTrack.getSettings();
        if (settings.width != null) {
            size.width = settings.width;
        }
        if (settings.height != null) {
            size.height = settings.height;
        }
        return size;
    }
}
