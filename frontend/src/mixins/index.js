export default {
    methods: {
        timeSince(date) {
            const seconds = Math.floor(((+new Date() / 1000) - date));
            let interval = Math.floor(seconds / 31536000);
            if (interval >= 1) {
                return `${interval} year${(interval > 1 ? 's' : '')}`;
            }
            interval = Math.floor(seconds / 2592000);
            if (interval >= 1) {
                return `${interval} month${(interval > 1 ? 's' : '')}`;
            }
            interval = Math.floor(seconds / 86400);
            if (interval >= 1) {
                return `${interval} day${(interval > 1 ? 's' : '')}`;
            }
            interval = Math.floor(seconds / 3600);
            if (interval >= 1) {
                return `${interval} hour${(interval > 1 ? 's' : '')}`;
            }
            interval = Math.floor(seconds / 60);
            if (interval >= 1) {
                return `${interval} minute${(interval > 1 ? 's' : '')}`;
            }
            return `${Math.floor(seconds)} seconds`;
        },
        // get string of file size in bytes
        fileSize(size) {
            if (!size) size = 0;
            let sizeString = `${(size).toFixed(2)} B`;

            if (size / 1000000000 < 1000) sizeString = `${(size / 1000000000).toFixed(2)} GB`;
            if (size / 1000000 < 1000) sizeString = `${(size / 1000000).toFixed(2)} MB`;
            if (size / 1000 < 1000) sizeString = `${(size / 1000).toFixed(2)} KB`;

            return sizeString;
        },
        urlSafe(str) {
            return str.toLowerCase().replace(' ', '-');
        },
        titleCase(str) {
            var splitStr = str.toLowerCase().split('-');
            for (var i = 0; i < splitStr.length; i++) {
                // You do not need to check if i is larger than splitStr length, as your for does that for you
                // Assign it back to the array
                splitStr[i] = splitStr[i].charAt(0).toUpperCase() + splitStr[i].substring(1);
            }
            // Directly return the joined string
            return splitStr.join(' ');
        }
    }
}