<template>
  <aside>
    <h2 class="sr-only">Details</h2>
    <div class="space-y-5">
      <div class="detail-row">
        <UploadIcon class="w-5 h-5 text-green-500"/>
        <span class="text-sm font-medium text-green-700">{{ torrent.seeders }} seeders</span>
      </div>
      <div class="detail-row">
        <DownloadIcon class="w-5 h-5 text-red-500"/>
        <span class="text-sm font-medium text-red-500">{{ torrent.leechers }} leechers</span>
      </div>
      <div class="detail-row">
        <DocumentIcon class="w-5 h-5 text-gray-400"/>
        <span class="text-sm font-medium text-gray-900">{{ fileSize(torrent.file_size) }}</span>
      </div>
      <div class="detail-row">
        <UserIcon class="w-5 h-5 text-gray-400"/>
        <span class="text-sm font-medium text-gray-900">{{ torrent.uploader }}</span>
      </div>
      <div class="detail-row">
        <CalendarIcon class="w-5 h-5 text-gray-400"/>
        <span class="text-sm font-medium text-gray-900">
          Uploaded on
          <time :datetime="torrent.upload_date * 1000">{{ new Date(torrent.upload_date * 1000).toLocaleDateString() }}</time>
        </span>
      </div>
    </div>
<!--    <div class="mt-6 bar-section">-->
<!--      <div>-->
<!--        <h2 class="text-sm font-medium text-gray-500">Categories</h2>-->
<!--        <ul role="list" class="mt-2 space-x-1 leading-8">-->
<!--          <li-->
<!--              class="inline"-->
<!--          >-->
<!--            <router-link-->
<!--                :to="{name: 'CategoryDetail', params: {name: urlSafe(this.category().name)}}"-->
<!--                class="inline-flex rounded-full border border-gray-300 px-3 py-0.5"-->
<!--            >-->
<!--              <div class="text-sm font-medium text-gray-900">{{ this.category().name }}</div>-->
<!--            </router-link>-->
<!--          </li>-->
<!--        </ul>-->
<!--      </div>-->
<!--    </div>-->

    <!-- Edit/Delete buttons -->
    <div class="mt-6 bar-section">
      <div class="inline-flex justify-between w-full">
        <button type="button" @click="downloadTorrent"
                class="text-white bg-green-600 border-transparent shadow-sm button hover:bg-green-700">
          <DownloadIcon class="mr-2 -ml-1 w-5 h-5"/>
          Torrent file
        </button>
<!--        <button type="button"-->
<!--                class="user-buttons text-gray-700 bg-white border-gray-300 hover:bg-gray-200">-->
<!--          <PencilIcon class="mr-2 -ml-1 w-5 h-5 text-gray-400"/>-->
<!--          <span>Edit</span>-->
<!--        </button>-->


<!--        <button type="button"-->
<!--                class="user-buttons text-white bg-red-600 border-transparent shadow-sm hover:bg-red-700">-->
<!--          <TrashIcon class="mr-2 -ml-1 w-5 h-5"/>-->
<!--          Delete-->
<!--        </button>-->
      </div>

    </div>
  </aside>
</template>

<script>
import {CalendarIcon, DocumentIcon, DownloadIcon, UploadIcon, UserIcon} from "@vue-hero-icons/outline";
import {mapState} from "vuex";
import HttpService from "@/common/http-service";

export default {
  name: "DetailsSidebar",
  components: {CalendarIcon, DocumentIcon, DownloadIcon, UploadIcon, UserIcon},
  props: {
    torrent: Object,
  },
  computed: {
    ...mapState(['categories']),
  },
  methods: {
    downloadTorrent() {
      //window.open(`${process.env.VUE_APP_API_BASE_URL}/torrent/download`, '_blank');
      HttpService.getBlob(`/torrent/download/${this.torrent.torrent_id}`, (res) => {
        const url = window.URL.createObjectURL(new Blob([res.data]));
        const link = document.createElement('a');
        link.href = url;
        link.setAttribute('download', `${this.torrent.title}.torrent`);
        document.body.appendChild(link);
        link.click();
      });
    }
  },
}
</script>

<style scoped>
.user-buttons {
  @apply inline-flex justify-center px-4 py-2 text-sm font-medium rounded-md border shadow-sm;
}

.bar-section {
  @apply py-6 space-y-8 border-t border-gray-200;
}

.detail-row {
  @apply flex items-center space-x-2;
}

.button {
  @apply inline-flex justify-center px-4 py-2 text-sm font-medium rounded-md border shadow-sm;
}
</style>
